use scryfall::card::Rarity;
use scryfall::search::prelude::*;
use scryfall::Card;
use scryfall::Result;

fn main() {
    // The search() function is async meaning it actually returns a Future, just like
    // async JS functions. Rust Futures are basically the same thing as JS Promises.
    // You need something that will actually schedule and execute the Future.
    // This is an async runtime for that purpose.
    let rt = tokio::runtime::Builder::new_current_thread()
        .enable_all()
        .build()
        .unwrap();

    // Just block on the main thread until the Future resolves.
    // Then unwrap the Result (panic if it's an Err)
    let card = rt.block_on(search()).unwrap();
    println!("{}", card.name);
}

// Return type is scryfall::Result<T>, which is an alias for Result::<T, scryfall::error::Error>,
// with T being scryfall::Card
async fn search() -> Result<Card> {
    // Sample query and search from the scryfall docs
    Query::And(vec![
        Query::Or(vec![power(9), toughness(9)]),
        Query::Custom("t:eldrazi".to_string()),
        set("bfz"),
        rarity(Rarity::Mythic),
        CardIs::OddCmc.into(),
    ])
    .search()
    // Search makes a request so it's async and you have to await.
    // Question mark is syntactic sugar. This Future returns a Result. Question mark makes it so
    // that if the Result is an Ok variant, the code will proceed with the value inside the Ok,
    // and if it's an Err variant, it will early return from the whole function with the Err.
    .await?
    // Get the first card from the iterator
    .next()
    // next() is async because the iterator supports pagination and may make a new request
    // for the next page.
    .await
    // next() returns an Option cause the iter might be empty. For simplicity, just unwrap
    // it and panic if it's None.
    // (You could alternatively convert the Option into a Result and then use a ?, so the
    // whole function would return Err if no card is found.)
    .unwrap()
    // Since there's no semicolon, this is all an expression, not a statement, and the function
    // will return the final Result<Card>
}
