import json
import os
import sys
from google import genai

with open('watchlist.json', 'r', encoding='utf-8') as f:
    watchlist = json.load(f)

movies_list = []
for movie in watchlist.get('movies', []):
    title = movie.get('title', 'Inconnu')
    year = movie.get('year', '') 
    movies_list.append(f"- {title} ({year})")

movies_text = "\n".join(movies_list)

envie = sys.argv[1] if len(sys.argv) > 1 else input("🍿 Quelle est ton envie de film aujourd'hui ? : ")

full_prompt = f"""Tu es un expert en cinéma passionné et pointu. Voici la liste complète de ma watchlist :
{movies_text}

Mon envie du moment : '{envie}'

Choisis exactement 3 films de cette liste qui correspondent le mieux à cette envie.
Pour chaque film, réponds avec ce format :

🎬 [Titre] ([Année])
→ [2-3 phrases max qui expliquent pourquoi ce film correspond parfaitement à l'envie, sans spoiler]

Règles strictes :
- Les 3 films doivent absolument être dans la liste fournie, aucune exception
- Ne recommande pas de films hors liste
- Classe-les du plus au moins pertinent
- Sois précis et personnel dans tes explications, pas générique"""

client = genai.Client()

print("🤖 L'IA analyse tes films, patiente quelques secondes...")
response = client.models.generate_content(
    model= "gemini-2.5-flash",
    contents = full_prompt
)

print("\n🎬 --- LA RECOMMANDATION --- 🎬")
print(response.text)
