#include "env.h"
#include "ui.h"
#include "utils/file_utils.h"
#include "dragon_utils/dragon_utils.h"
#include <stdio.h>
#include <stdlib.h>
#include <gtk/gtk.h>

#define GLADE_PATH "./resources/ui/Window.glade"

int main(int argc, char ** argv) {
    FILE* handle = open_ec();

    if (handle == NULL) {
        // return -1;
    }

    ui_t *ui = ui_init(handle, GLADE_PATH, argc, argv);

    gtk_widget_show(ui->window);
    gtk_main();

    g_object_unref(ui->builder);

    close_ec(handle);
    return 0;
}
