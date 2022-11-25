package main
// file should be an simple text file with int values in it. an software will plot an graph
import (
    grob "github.com/MetalBlueberry/go-plotly/graph_objects"
    "github.com/MetalBlueberry/go-plotly/offline"
	"bufio"
    "fmt"
    "os"
	"strings"
	"strconv"
)
func read_Lines() (message []int) {
	array := []int{}
	filePath := "input.csv"
    readFile, err := os.Open(filePath)
    if err != nil {
        fmt.Println(err)
    }
    fileScanner := bufio.NewScanner(readFile)
    fileScanner.Split(bufio.ScanLines)
    var fileLines []string
    for fileScanner.Scan() {
        fileLines = append(fileLines, fileScanner.Text())
    }
    readFile.Close()
    for _, line := range fileLines {
		res1 := strings.Trim(line, " ")
		res2 := strings.Replace(res1,"HZ","",2)
		res3 := strings.Replace(res2," ","",2)
		freq, err := strconv.Atoi(res3)
		if(freq !=0) {
			array = append(array, freq)
		}		
		if err != nil {	
		}
    }   
	return array;
}
func data_plot(array []int){
	fig := &grob.Fig{
        Data: grob.Traces{
            &grob.Bar{
                Type: grob.TraceTypeScatter,
                Y:    array,
            },
        },
        Layout: &grob.Layout{
            Title: &grob.LayoutTitle{
                Text: "Doing really dumb things. (Using RPI for Real Time)",
            },
        },
    }

    offline.Show(fig)
}
func main() {
	array := read_Lines();
    data_plot(array);
}
