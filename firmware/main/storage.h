#pragma once
#include "esp_err.h"

typedef struct { char status[8]; } display_state_t;
esp_err_t storage_load(display_state_t *state);
esp_err_t storage_save(const display_state_t *state);
