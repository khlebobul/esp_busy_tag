#include "nvs_flash.h"
#include "display.h"
#include "protocol.h"
#include "storage.h"
#include "usb.h"
#include "button.h"
#include "bsp/esp-bsp.h"
#include "driver/i2c_master.h"
#include "esp_io_expander.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"
#include <string.h>

static void release_v2_panel_reset(void) {
    if (bsp_i2c_init() != ESP_OK || i2c_master_probe(bsp_i2c_get_handle(), BSP_IO_EXPANDER_I2C_ADDRESS, 50) != ESP_OK) return;
    esp_io_expander_handle_t expander = bsp_io_expander_init();
    if (!expander) return;
    const uint32_t pins = IO_EXPANDER_PIN_NUM_0 | IO_EXPANDER_PIN_NUM_1 | IO_EXPANDER_PIN_NUM_2;
    ESP_ERROR_CHECK(esp_io_expander_set_dir(expander, pins, IO_EXPANDER_OUTPUT));
    ESP_ERROR_CHECK(esp_io_expander_set_level(expander, pins, 1));
    vTaskDelay(pdMS_TO_TICKS(100));
    ESP_ERROR_CHECK(esp_io_expander_set_level(expander, pins, 0));
    vTaskDelay(pdMS_TO_TICKS(300));
    ESP_ERROR_CHECK(esp_io_expander_set_level(expander, pins, 1));
}

void app_main(void) {
    nvs_flash_init(); display_state_t state; strcpy(state.status, "FREE");
    release_v2_panel_reset();
    display_init(&state); protocol_init(&state); usb_start(); button_start();
}
