#include "env.h"
#include "utils/file_utils.h"
#include "dragon_utils/dragon_utils.h"
#include <stdio.h>
#include <stdlib.h>
#include <gtk/gtk.h>


int main(int argc, char ** argv) {
    /* FILE* handle = open_ec();

    set_battery_threshold(handle, 100);
    set_cooler_boost(handle, 0x0);

    close_ec(handle);*/

    GtkBuilder* builder;
    GtkWidget*  window;

    gtk_init(&argc, &argv);

    builder = gtk_builder_new_from_file("resources/ui/Window.glade");

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
