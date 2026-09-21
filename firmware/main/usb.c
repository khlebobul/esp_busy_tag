#include "usb.h"
#include "protocol.h"
#include "driver/usb_serial_jtag.h"
#include "freertos/FreeRTOS.h"
#include "freertos/task.h"

static void usb_task(void *unused) {
    char line[512]; size_t length = 0;
    for (;;) { char c; int read = usb_serial_jtag_read_bytes(&c, 1, portMAX_DELAY); if (read != 1) continue;
        if (c == '\n') { line[length] = 0; if (length) protocol_handle(line); length = 0; }
        else if (c != '\r' && length < sizeof(line) - 1) line[length++] = c;
        else if (length == sizeof(line) - 1) length = 0;
    }
}
void usb_start(void) { usb_serial_jtag_driver_install(&(usb_serial_jtag_driver_config_t){ .rx_buffer_size = 1024, .tx_buffer_size = 1024 }); xTaskCreate(usb_task, "usb", 4096, NULL, 5, NULL); }
