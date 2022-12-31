import requests
from bs4 import BeautifulSoup
import sys
import os

INDEX_URL = "http://classics.mit.edu"

# Browse/browse-Plato.html

def main(author, works):
    r = requests.get(INDEX_URL + "/Browse/index.html") # http://classics.mit.edu/Browse/index.html
    soup = BeautifulSoup(r.text, 'html.parser')

    author_url = fetch_author_url(soup, author)
    if author_url:
        r = requests.get(INDEX_URL + "/Browse/" + author_url) # http://classics.mit.edu/Browse/browse-[AUTHOR].html
        soup = BeautifulSoup(r.text, 'html.parser')

        # extract the author name that the website internally uses in its html:
        author_html = author_url.split('-')[1].split('.')[0]
        literature = fetch_literature_urls(soup, author_html)
    else:
        print("Author not found ...")
        return
    
    current_dir = os.getcwd()
    new_dir = os.path.join(current_dir, author)
    if not os.path.isdir(new_dir):
        os.mkdir(new_dir)

    work_html_dict = fetch_works_htmls(literature, works)
    for book_name in work_html_dict:
        new_file = author + "/" + book_name + ".txt"
        if os.path.exists(os.path.join(current_dir, new_file)):
            print("'" + new_file + "'" + " already exists!")
            continue

        r = requests.get(INDEX_URL + work_html_dict[book_name])
        soup = BeautifulSoup(r.text, 'html.parser')
        text_file = next(filter(lambda l: '.txt' in l['href'] and book_name in l['href'], soup.find_all('a',  href=True))).get('href')
        r = requests.get(INDEX_URL + '/' + author_html + '/' + text_file)
        with open(new_file, 'w') as f:
            f.write(r.text)
            print("'" + new_file + "'" + " created")

    print("Done!")


def fetch_author_url(soup, author):
    for link in soup.find_all('a'):
        if author in link['href'].lower() or author in link.text.lower():
            return link['href']
    return ""

def fetch_literature_urls(soup, author_html):
    match_string = '/' + author_html + '/'
    works = filter(lambda l: match_string in l.get('href'), soup.find_all('a'))
    return works

def fetch_works_htmls(literature, demanded_works):
    work_html_dict = {}
    if len(demanded_works) > 0:
        for link in literature:
            name = link['href'].split('/')[2].split('.')[0].lower()
            if name in demanded_works:
                work_html_dict[name] = link['href']
    else:
        for link in literature:
            name = link['href'].split('/')[2].split('.')[0].lower()
            work_html_dict[name] = link['href']

    return work_html_dict

# USAGE:
# 'py fetcher.py plato phaedrus republic symposium'
# => Downloads "Phaedrus", "The Republic" and "Symposium" written by Plato
# OBS:
# if omitting works, e.g:
# 'py fetcher.py plato'
# => all available literature by Plato will be downloaded.
if __name__ == "__main__":
    args = sys.argv[1:]
    if args:
        author = args[0].lower()
        works = args[1:]
        if works:
            for i in range(0, len(works)):
                works[i] = works[i].lower()
        main(author, works)
    else:
        print("Please provide an author name")
