#pragma once
#include "storage.h"
void protocol_init(const display_state_t *initial);
void protocol_handle(const char *line);
void protocol_cycle_status(void);
