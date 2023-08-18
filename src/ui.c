#include "ui.h"
#include <malloc.h>
#include "dragon_utils/dragon_utils.h"
#include "env.h"

void init_battery_buttons(ui_t *ui) {
    char battery = get_battery_threshold(ui->ec_handle);
    if (battery == -1)
        return;
    GtkRadioButton *current = NULL;
    if (battery == BATTERY_BATTERY) {
        current = ui->battery.best_for_battery;
    } else if (battery == BATTERY_HYBRID) {
        current = ui->battery.hybrid;
    } else {
        current = ui->battery.best_for_mobility;
    }

    gtk_toggle_button_set_active(GTK_TOGGLE_BUTTON(current), (gboolean)1);
}

ui_t *ui_init(FILE *ec_handle, const char *path, int argc, char **argv) {
    ui_t *res = malloc(sizeof(ui_t));
    if (res == NULL) {
        return NULL;
    }

    gtk_init(&argc, &argv);

    res->builder = gtk_builder_new_from_file(path);
    res->ec_handle = ec_handle;

    res->window = GTK_WIDGET(gtk_builder_get_object(res->builder, "window_main"));

    res->battery.best_for_mobility = GTK_RADIO_BUTTON(gtk_builder_get_object(res->builder, "battery_mobility"));
    res->battery.hybrid = GTK_RADIO_BUTTON(gtk_builder_get_object(res->builder, "battery_hybrid"));
    res->battery.best_for_battery = GTK_RADIO_BUTTON(gtk_builder_get_object(res->builder, "battery_battery"));
    gtk_builder_connect_signals(res->builder, res);

    init_battery_buttons(res);

    return res;
}


void on_battery_changed(GtkRadioButton *self, gpointer user_data) {
    if (!gtk_toggle_button_get_active(GTK_TOGGLE_BUTTON(self)))
        return;
    ui_t *ui = (ui_t *) user_data;
    if (self == ui->battery.best_for_mobility) {
        printf("best for mobility\n");
        set_battery_threshold(ui->ec_handle, BATTERY_MOBILITY);
    } else if (self == ui->battery.hybrid) {
        printf("hybrid\n");
        set_battery_threshold(ui->ec_handle, BATTERY_HYBRID);
    } else if (self == ui->battery.best_for_battery) {
        printf("best for battery\n");
        set_battery_threshold(ui->ec_handle, BATTERY_BATTERY);
    }
}

void on_window_main_destroy() {
    gtk_main_quit();
}

