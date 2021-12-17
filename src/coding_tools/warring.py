

string = """\

<style>
@import url('https://fonts.googleapis.com/css2?family=Roboto:wght@100;300&display=swap');
</style>
<style>

th, td {
  padding: 15px;
  text-align: left;
  border-bottom: 1px solid #ddd;
  font-family: 'Roboto', sans-serif;
}

tr:nth-child(even) {
background-color: #f2f2f2; 

}

th{
    background-color: #04AA6D;
    border-radius : 5px;
      font-family: 'Roboto', sans-serif;
}


</style>
"""


lines=open("build.txt","r").readlines()
output=open("index.html","a")
output.write(string)
z=0
z=0
output_warrings=[]
output_suggestions=[]
to_do=[]


output.write("<table> \n   <tr>     <th>Warring message</th>  <th>Line</th> <th>Suggestion</th></tr>")
for a in lines:
	print(a)

	if(a.find("warning")> -1):
		output_warrings.append(a)
	if(a.find("-->") > -1):
		output_suggestions.append(a)
	if(a.find("help:") > -1):
		to_do.append(a)





for a in range(0,len(to_do)):
	output.write("<tr>")

	output.write("<td>")
	output.write(output_warrings[a])
	output.write("</td>")
	output.write("<td>")
	output.write(output_suggestions[a])
	output.write("</td>")
	output.write("<td>")
	output.write(to_do[a])
	output.write("</td>")
	output.write("</tr>")

	
