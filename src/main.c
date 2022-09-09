#include "headers/env.h"
#include "headers/file_utils.h"
#include "headers/dragon_utils.h"
#include <stdio.h>
#include <stdlib.h>
#include <gtk/gtk.h>


static void print_hello(GtkWidget *widget, gpointer user_data) {
	g_print("Hello, World!\n");
}

static void activate (GtkApplication *app, gpointer user_data) {
	GtkWidget *window;
	GtkWidget *button;

	window = gtk_application_window_new(app);
	gtk_window_set_title(GTK_WINDOW(window), "window");
	gtk_window_set_default_size(GTK_WINDOW(window), 200, 200);

	button = gtk_button_new_with_label("Hello, World!\n");
	g_signal_connect(button, "clicked", G_CALLBACK(print_hello), NULL);
	gtk_window_set_child(GTK_WINDOW(window), button);

	gtk_window_present(GTK_WINDOW(window));

}


int main(int argc, char ** argv) {

	FILE * handle = open_ec();

	set_battery_threshold(handle, 50);

	close_ec(handle);
	
	GtkApplication *app;
	int status;

	app = gtk_application_new("com.example", G_APPLICATION_FLAGS_NONE);
	g_signal_connect(app, "activate", G_CALLBACK(activate), NULL);
	status = g_application_run(G_APPLICATION(app), argc, argv);
	g_object_unref(app);


	return status;
}


