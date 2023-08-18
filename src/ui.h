#ifndef DRAGON_CENTER_UI_H
#define DRAGON_CENTER_UI_H

#include <gtk/gtk.h>
#include "dragon_utils/dragon_utils.h"

void on_battery_changed(GtkRadioButton *self, gpointer user_data);

typedef struct {
    struct {
        GtkRadioButton *best_for_mobility;
        GtkRadioButton *hybrid;
        GtkRadioButton *best_for_battery;
    } battery;

    GtkBuilder *builder;
    GtkWidget *window;

    FILE *ec_handle;
} ui_t;

ui_t *ui_init(FILE *ec_handle, const char *path, int argc, char **argv);

void on_window_main_destroy();

#endif /* !DRAGON_CENTER_UI_H */
