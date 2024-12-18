document.getElementById('fetchQuotes').addEventListener('click', fetchQuotes);

async function fetchQuotes() {
    const response = await fetch('https://api.twitter.com/2/users/by/username/QuoteGuy08/tweets?max_results=5', {
        headers: {
            'Authorization': 'Bearer YOUR_TWITTER_BEARER_TOKEN'
        }
    });

    if (response.ok) {
        const data = await response.json();
        const quotesElement = document.getElementById('quotes');
        quotesElement.innerHTML = '';

        if (data.data) {
            data.data.forEach((tweet, index) => {
                const quoteElement = document.createElement('p');
                quoteElement.textContent = `Quote ${index + 1}: ${tweet.text}`;
                quotesElement.appendChild(quoteElement);
            });
        } else {
            quotesElement.textContent = 'No quotes found.';
        }
    } else {
        console.error('Error fetching quotes:', response.statusText);
    }
}
