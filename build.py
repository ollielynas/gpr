

games = {
    "tic_tac_toe": ["T", "Tic Tac Toe"],
    "error": ["E", "Error"],
}

html_template = ""
with open("template.html", "r") as my_file:
    html_template = my_file.read()


game_links = ""


for i in games:
    html = html_template
    
    html = html.replace("TITLE", i)
    html = html.replace("NAME", games[i][1])
    
    
    with open(f"games/{i}.html", "w") as my_file:
        my_file.write(html)

    if i != "error":
        game_links += f"<a href='https://ollielynas.github.io/gpr/games/{i}.html?'>a<img src='https://ollielynas.github.io/gpr/img/{i}.png' alt='start game of {games[i][1]}'></img></a>"

index = ""
with open("index.html", "r") as my_file:
    index = my_file.read().split("<!-- GAMES -->")

with open("index.html", "w") as my_file:
    index[1] = game_links
    index = my_file.write("<!-- GAMES -->".join(index))
