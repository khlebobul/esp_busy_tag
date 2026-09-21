#include "storage.h"
#include <string.h>
#include "nvs.h"

esp_err_t storage_load(display_state_t *state) {
    strcpy(state->status, "FREE");
    nvs_handle_t nvs; if (nvs_open("display", NVS_READONLY, &nvs) != ESP_OK) return ESP_OK;
    size_t size = sizeof(state->status); nvs_get_str(nvs, "status", state->status, &size);
    nvs_close(nvs); return ESP_OK;
}

esp_err_t storage_save(const display_state_t *state) {
    nvs_handle_t nvs; esp_err_t err = nvs_open("display", NVS_READWRITE, &nvs); if (err != ESP_OK) return err;
    err = nvs_set_str(nvs, "status", state->status);
    if (err == ESP_OK) err = nvs_commit(nvs);
    nvs_close(nvs);
    return err;
}
