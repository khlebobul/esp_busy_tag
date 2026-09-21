#include "protocol.h"
#include "display.h"
#include "storage.h"
#include <stdio.h>
#include <string.h>
#include "cJSON.h"
#include "driver/usb_serial_jtag.h"

static display_state_t state;
void protocol_init(const display_state_t *initial) { state = *initial; }
static void reply(const char *line) { usb_serial_jtag_write_bytes(line, strlen(line), 100); }

void protocol_handle(const char *line) {
    cJSON *json = cJSON_Parse(line); if (!json) { reply("{\"error\":\"invalid json\"}\n"); return; }
    cJSON *cmd = cJSON_GetObjectItemCaseSensitive(json, "cmd");
    if (!cJSON_IsString(cmd)) goto bad;
    if (!strcmp(cmd->valuestring, "info")) reply("{\"device\":\"desk-display\",\"version\":\"0.1.0\",\"board\":\"esp32-s3-touch-amoled-1.8\"}\n");
    else if (!strcmp(cmd->valuestring, "get_state")) { char buf[64]; snprintf(buf, sizeof(buf), "{\"status\":\"%s\"}\n", state.status); reply(buf); }
    else if (!strcmp(cmd->valuestring, "status")) { cJSON *v = cJSON_GetObjectItemCaseSensitive(json, "value"); if (!cJSON_IsString(v) || (strcmp(v->valuestring,"free") && strcmp(v->valuestring,"busy") && strcmp(v->valuestring,"meeting"))) goto bad; strcpy(state.status, v->valuestring); for (char *c = state.status; *c; c++) if (*c >= 'a' && *c <= 'z') *c -= 'a' - 'A'; display_show(&state); storage_save(&state); reply("{\"ok\":true}\n"); }
    else goto bad;
    cJSON_Delete(json); return;
bad: cJSON_Delete(json); reply("{\"error\":\"invalid command\"}\n");
}

void protocol_cycle_status(void) {
    if (!strcmp(state.status, "FREE")) strcpy(state.status, "BUSY");
    else if (!strcmp(state.status, "BUSY")) strcpy(state.status, "MEETING");
    else strcpy(state.status, "FREE");
    display_show(&state); storage_save(&state);
}
