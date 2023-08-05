#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <curl/curl.h>
#include <cjson/cJSON.h>

const char *API_KEY = "YOUR_API_KEY_HERE";

struct MemoryStruct {
 char *memory;
 size_t size;
};

static size_t WriteMemoryCallback(void *contents, size_t size, size_t nmemb, void *userp) {
 size_t realsize = size * nmemb;
 struct MemoryStruct *mem = (struct MemoryStruct *)userp;

 mem->memory = realloc(mem->memory, mem->size + realsize + 1);
 if (mem->memory == NULL) {
   printf("Not enough memory (realloc returned NULL)\n");
   return 0;
 }

 memcpy(&(mem->memory[mem->size]), contents, realsize);
 mem->size += realsize;
 mem->memory[mem->size] = 0;

 return realsize;
}

double get_stock_price(const char *ticker) {
 CURL *curl_handle;
 CURLcode res;

 struct MemoryStruct chunk;
 chunk.memory = malloc(1);
 chunk.size = 0;

 curl_global_init(CURL_GLOBAL_ALL);

 curl_handle = curl_easy_init();

 char url[256];
 snprintf(url, sizeof(url), "https://lnkd.in/euB4anxs", ticker, API_KEY);

 curl_easy_setopt(curl_handle, CURLOPT_URL, url);
 curl_easy_setopt(curl_handle, CURLOPT_WRITEFUNCTION, WriteMemoryCallback);
 curl_easy_setopt(curl_handle, CURLOPT_WRITEDATA, (void *)&chunk);

 res = curl_easy_perform(curl_handle);
 if (res != CURLE_OK) {
   fprintf(stderr, "curl_easy_perform() failed: %s\n", curl_easy_strerror(res));
 } else {
   cJSON *json = cJSON_Parse(chunk.memory);
   if (json == NULL) {
     printf("Error parsing JSON\n");
   } else {
     cJSON *global_quote = cJSON_GetObjectItemCaseSensitive(json, "Global Quote");
     cJSON *price = cJSON_GetObjectItemCaseSensitive(global_quote, "05. price");
     if (cJSON_IsString(price) && price->valuestring != NULL) {
       double stock_price = strtod(price->valuestring, NULL);
       cJSON_Delete(json);
       free(chunk.memory);
       curl_easy_cleanup(curl_handle);
       curl_global_cleanup();
       return stock_price;
     }
     cJSON_Delete(json);
   }
 }

 free(chunk.memory);
 curl_easy_cleanup(curl_handle);
 curl_global_cleanup();

 return -1;
}

int main() {
 char ticker[10];
 printf("Enter the stock ticker: ");
 scanf("%s", ticker);

 double stock_price = get_stock_price(ticker);

 if (stock_price != -1) {
   printf("The current stock price of %s is: $%.2f\n", ticker, stock_price);
 } else {
   printf("Error fetching stock price.\n");
 }
