package app; /**
 * Sample Skeleton for 'main.fxml' Controller Class
 */

import java.awt.*;
import java.io.BufferedReader;
import java.io.File;
import java.io.IOException;
import java.io.InputStreamReader;
import java.net.MalformedURLException;
import java.net.URL;
import java.net.URLConnection;
import java.util.ResourceBundle;

import javafx.application.Platform;
import javafx.fxml.FXML;
import javafx.scene.chart.LineChart;
import javafx.scene.chart.XYChart;
import javafx.scene.input.MouseEvent;
import org.ini4j.Wini;

import javax.swing.*;


public class Controller implements Runnable {
    XYChart.Series x = new XYChart.Series();
    XYChart.Series y = new XYChart.Series();
    XYChart.Series z = new XYChart.Series();
    private int val;


    @FXML // ResourceBundle that was given to the FXMLLoader
    private ResourceBundle resources;

    @FXML // URL location of the FXML file that was given to the FXMLLoader
    private URL location;

    @FXML // fx:id="graph"
    private LineChart<Number, Number> graph; // Value injected by FXMLLoader

    @FXML
    void config(MouseEvent event) throws IOException {
        Desktop main = Desktop.getDesktop();
        main.open(new File("gyro.config"));

    }

    @FXML
    void connect(MouseEvent event) throws IOException {
        new Thread(this).start();

    }

    @FXML
    void settings(MouseEvent event) throws IOException {
        var ip = JOptionPane.showInputDialog(null, "Enter IP");
        var enter_mode = JOptionPane.showInputDialog(null, "Enter Mode");
        Wini ini = new Wini(new File("gyro.config"));
        ini.put("ip", "ip", ip);
        ini.put("ip", "enter_mode", enter_mode);
        ini.store();
    }

    @FXML
    void turn_off(MouseEvent event) {
        System.exit(0);
    }

    @FXML
        // This method is called by the FXMLLoader when initialization is complete
    void initialize() {
        assert graph != null : "fx:id=\"graph\" was not injected: check your FXML file 'main.fxml'.";

    }

    public String data(String ip) throws IOException {
        URL oracle = new URL(ip);
        URLConnection yc = oracle.openConnection();
        BufferedReader in = new BufferedReader(new InputStreamReader(
                yc.getInputStream()));
        String inputLine;
        while ((inputLine = in.readLine()) != null)
            return inputLine;
        in.close();
        return null;
    }

    @Override
    public void run() {

        while (true) {
            Wini ini = null;
            try {
                ini = new Wini(new File("gyro.config"));
            } catch (IOException e) {
                e.printStackTrace();
            }
            var ip = ini.get("ip", "ip");
            var mode = ini.get("ip", "enter_mode");
            try {
                var data = data(ip).split(",");
                Thread.sleep(100);

                x.setName("x");
                y.setName("y");
                z.setName("z");
                if (data == null) {
                } else {
                    System.out.println(data[0]);
                    //populating the series with data
                    x.getData().add(new XYChart.Data(String.valueOf(val), Integer.valueOf(data[0])));
                    y.getData().add(new XYChart.Data(String.valueOf(val), Integer.valueOf(data[1])));
                    z.getData().add(new XYChart.Data(String.valueOf(val), Integer.valueOf(data[2])));
                    Platform.runLater(() -> {
                        //graph.getData().retainAll();

                        graph.getData().add(x);
                        graph.getData().add(y);

                        graph.getData().add(z);




                    });

                    val++;


                }

            } catch (InterruptedException e) {
                e.printStackTrace();
            } catch (IOException e) {
                e.printStackTrace();
            }
        }
    }
}

