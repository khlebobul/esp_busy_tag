#include "button.h"
#include "protocol.h"
#include "driver/gpio.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

#define BUTTON_GPIO 0
#define BUTTON_POLL_MS 10
#define BUTTON_STABLE_MS 60

static void button_task(void *unused) {
    gpio_config_t io = {
        .pin_bit_mask = (1ULL << BUTTON_GPIO),
        .mode = GPIO_MODE_INPUT,
        .pull_up_en = GPIO_PULLUP_ENABLE,
        .pull_down_en = GPIO_PULLDOWN_DISABLE,
        .intr_type = GPIO_INTR_DISABLE,
    };
    gpio_config(&io);
    int debounced = 1, pending = 1, count = 0;
    for (;;) {
        vTaskDelay(pdMS_TO_TICKS(BUTTON_POLL_MS));
        int level = gpio_get_level(BUTTON_GPIO);
        if (level == pending) {
            count++;
            if (count * BUTTON_POLL_MS >= BUTTON_STABLE_MS && level != debounced) {
                if (debounced == 1 && level == 0) protocol_cycle_status();
                debounced = level;
            }
        }
        else { pending = level; count = 0; }
    }
}

void button_start(void) { xTaskCreate(button_task, "button", 3072, NULL, 4, NULL); }