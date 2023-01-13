#include "env.h"
#include "utils/file_utils.h"
#include "dragon_utils/dragon_utils.h"
#include <stdio.h>
#include <stdlib.h>
#include <gtk/gtk.h>

#define GLADE_PATH "./resources/ui/Window.glade"


int main(int argc, char ** argv) {
    /*FILE* handle = open_ec();

    set_battery_threshold(handle, 50);
    set_cooler_boost(handle, 0x0);

    close_ec(handle);*/

    printf("%s\n", argv[0]);

    GtkBuilder* builder;
    GtkWidget*  window;

    gtk_init(&argc, &argv);

    builder = gtk_builder_new_from_file(GLADE_PATH);

    window = GTK_WIDGET(gtk_builder_get_object(builder, "window_main"));
    gtk_builder_connect_signals(builder, NULL);

    g_object_unref(builder);

    gtk_widget_show(window);
    gtk_main();
    return 0;
}

void on_window_main_destroy()
{
    gtk_main_quit();
    printf("test\n");
}
