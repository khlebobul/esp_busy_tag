#include "display.h"
#include <stdio.h>
#include <string.h>
#include "bsp/esp-bsp.h"
#include "misc/lv_palette.h"

extern const lv_font_t lv_font_orbitron_48;

static lv_obj_t *title;
static const int DISPLAY_BRIGHTNESS = 70;

static lv_color_t bg_color_for_status(const char *status) {
    if (!strcmp(status, "BUSY")) return lv_palette_main(LV_PALETTE_RED);
    if (!strcmp(status, "MEETING")) return lv_palette_main(LV_PALETTE_BLUE);
    return lv_palette_main(LV_PALETTE_GREEN);
}

void display_init(const display_state_t *state) {
    lv_display_t *display = bsp_display_start();
    bsp_display_lock(0);
    bsp_display_rotate(display, LV_DISPLAY_ROTATION_90);
    bsp_display_brightness_set(DISPLAY_BRIGHTNESS);
    lv_obj_t *screen = lv_screen_active();
    lv_obj_set_style_bg_color(screen, bg_color_for_status(state->status), 0);
    title = lv_label_create(screen); lv_obj_set_style_text_color(title, lv_color_white(), 0); lv_obj_set_style_text_font(title, &lv_font_orbitron_48, 0); lv_obj_align(title, LV_ALIGN_CENTER, 0, 0);
    bsp_display_unlock(); display_show(state);
}

void display_show(const display_state_t *state) {
    bsp_display_lock(0);
    lv_obj_set_style_bg_color(lv_screen_active(), bg_color_for_status(state->status), 0);
    lv_label_set_text(title, state->status);
    bsp_display_unlock();
}
