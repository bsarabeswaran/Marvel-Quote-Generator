import './App.css';
import React from 'react';
import Button from 'react-bootstrap/Button'

class App extends React.Component {
  constructor(props) {
    super(props);
    this.state = {
      quote_list: []
    }
    this.get_quotes_and_change_list = this.get_quotes_and_change_list.bind(this);
  }

  get_quotes_and_change_list(){
    let {
      quote_list
    } = this.state;
    if(quote_list.length >= 5) {
      quote_list.shift();
    }
    fetch("http://127.0.0.1:5000/quote", {method: 'POST', mode: 'cors', credentials: 'same-origin', headers: 
                                                    { 'Content-Type': 'application/json',
                                                      'Content-Length': '15'}, body: JSON.stringify({ length: "20" })})
    .then((response) => {
      if(!response.ok) throw Error(response.statusText);
      return response.json();
    })
    .then((data) => {
      let quote = data['quote'];
      console.log(quote);
      let pBodyHolder = (
      <p id="quote-holder">
        {quote}
      </p>
      );
      let new_list = quote_list.concat(pBodyHolder);
      this.setState({
        quote_list: new_list
      });
    })
    .catch((error) => console.log(error));
  }

  render(){
    const {
      quote_list
    } = this.state;
    return (
      <div className="App">
        <header className="App-header">
          <p>
            Marvel Quote Generator
          </p>
          <Button onClick={this.get_quotes_and_change_list} variant="secondary"> Generate a Quote</Button>
          <div id="quotes-holder">
            {quote_list}
          </div>
          <a
            className="App-link"
            href="https://github.com/bsarabeswaran/Marvel-Quote-Generator"
          >
            View Source
          </a>
        </header>
      </div>
    );
  }
}

export default App;
