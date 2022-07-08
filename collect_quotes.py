import requests
import math
import re
from bs4 import BeautifulSoup

def main():
    movie_links = get_urls()

    with open("all_quotes.txt", "w") as f:
        for link in movie_links:
            link_html = requests.get("https://www.imdb.com"+link+"quotes/")
            link_soup = BeautifulSoup(link_html.content, "html.parser")

            quote_divs = link_soup.find_all("div", class_="sodatext")
            for quote in quote_divs:
                quote_lines = re.sub('\n\n+', '\n', re.sub('\:\n*', ': ', re.sub('\[[^[]*\]', '', quote.getText()))).strip()
                f.write(quote_lines)
                f.write('\n')


def get_urls():
    list_url = "https://www.imdb.com/list/ls042924161/"
    list_url_html = requests.get(list_url)
    list_url_soup = BeautifulSoup(list_url_html.content, "html.parser")

    num_titles_info = list_url_soup.find(class_="lister-total-num-results")
    num_titles = int(num_titles_info.text.strip().replace(" titles", ""))
    if num_titles > 100:
        num_pages = math.ceil(num_titles / 100)
    else:
        num_pages = 1
    
    list_of_titles = []

    results = list_url_soup.find_all("h3", class_="lister-item-header")
    for result in results:
        a_tag = result.find('a')
        list_of_titles.append(a_tag['href'])

    for page_no in range(1, num_pages):
        list_url = "https://www.imdb.com" + list_url_soup.find('a', class_="lister-page-next")['href']
        list_url_html = requests.get(list_url)
        list_url_soup = BeautifulSoup(list_url_html.content, "html.parser")
        results = list_url_soup.find_all("h3", class_="lister-item-header")
        for result in results:
            a_tag = result.find('a')
            list_of_titles.append(a_tag['href'])
    
    return list_of_titles

        

if __name__ == '__main__':
    main()