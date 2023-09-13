## Introduction

## Methodology

### Obtaining data

#### Crawling tool

We use a Rust recursive scraper developed by a DKU student Sichang He. For a more comprehensive understanding, the documentation and source code can be found **[here](https://github.com/SichangHe/scraper)**.

To make configuation and run the scraper, we wrote a python script `scrape.py` to manuplate the scraper. The file is attached in the submission.
#### Crawling Strategy

Our approach to website crawling is centered on **Domain Names**. Our main focus is on all websites within the `.dukekunshan.edu.cn` domain.

However, we don't want to limit our knowledge base solely to DKU's domain. To that end, we also crawl the **outer ring**. The outer ring refers to all webpages that are directly linked from DKU's internal pages. Currently, **our ring depth is set to 1**, which means we don't crawl further into pages that are linked by these outer ring pages.

### Data processing

#### Data Cleaning and Tokenization

Since our raw data is the raw html file, it is hard to directly process the html with bolierplate dressing. So, we first use an content extractor to extract the main pure text content of the html and then apply tokenization to it.

The content extractor we use is **[peduncle](https://github.com/midstreeeam/peduncle/tree/main)**, a DOM based extractor developed by Kaiyuan Lou. The main determine condition for a html node to be seen as "main content" is measured by text-children ratio, which is calculated by the formular:
$$
  \frac{t\times(1+c/n)}{c/n}
$$
Where $t$ is the text length, $c$ is the total number of child nodes, and $n$ is a adjustable variable.

For tokenization, we use **[jieba](https://github.com/fxsjy/jieba)** and **[nltk](https://www.nltk.org/)** for Chinese and English tokenization, respectively.

#### Pagerank

PageRank is used to measure the importance of documented we scraped. The formular is like this:
$$
PR(p) = (1-d) + d \left( \frac{{PR(t_1)}}{{C(t_1)}} + \frac{{PR(t_2)}}{{C(t_2)}} + \ldots + \frac{{PR(t_n)}}{{C(t_n)}} \right)
$$
$PR(p)$ represents the PageRank of a given page $p$. $PR(t_i)$ refers to the PageRank of the pages $t_i$ that link to page $p$, while $C(t_i)$ represents the number of outbound links from page $t_i$. The parameter $d$ is the damping factor.

## Discussion

## Conclusion
