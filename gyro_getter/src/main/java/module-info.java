module GyroGetter.app.Main  {
    requires javafx.base;
    requires javafx.graphics;
    requires javafx.controls;
    requires  javafx.fxml;
    requires com.jfoenix;
    requires java.desktop;
    requires ini4j;
    opens   app to javafx.fxml;
    exports app to javafx.graphics;


}