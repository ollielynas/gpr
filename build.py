

games = {
    "tic_tac_toe": ["T", "Tic Tac Toe"],
    "error": ["E", "Error"],
}

html_template = ""
with open("template.html", "r") as my_file:
    html_template = my_file.read()



for i in games:
    html = html_template
    
    html = html.replace("TITLE", i)
    html = html.replace("NAME", games[i][0])
    
    
    with open(f"games/{i}.html", "w") as my_file:
        my_file.write(html)