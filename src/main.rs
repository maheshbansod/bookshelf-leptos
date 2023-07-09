use leptos::{html::Form, *};

mod bookshelf;
use bookshelf::*;
use serde::{Deserialize, Serialize};
fn main() {
    mount_to_body(move |cx| view! {cx, <Bookshelf /> })
}

#[component]
fn Bookshelf(cx: Scope) -> impl IntoView {
    let (bookshelf, set_bookshelf) = create_signal(cx, Bookshelf::new());
    let on_add_book = move |book: Book| {
        set_bookshelf.update(|bookshelf| bookshelf.add_book(book));
    };
    view! {cx,
        <BookSearch on_add_book=on_add_book/>
        <h1>"Bookshelf"</h1>
        <hr/>
        <input type="text" placeholder="Search books in the bookshelf" />
        <For
            each=move || bookshelf().books().clone()
            key=|book| book.id().clone()
            view=move |cx, book| view! {cx, <Book book />}
                />
    }
}

#[component]
fn BookSearch<F>(cx: Scope, on_add_book: F) -> impl IntoView
where F: Fn(Book) + 'static + Copy {
    let (query, set_query) = create_signal(cx, String::new());
    let (is_searching, set_is_searching) = create_signal(cx, false);
    let search = move |_query: String| {
        set_is_searching.set(true);
    };
    view! {cx,
        <input
            type="search"
            placeholder="Search books"
            on:input=move |e| set_query(event_target_value(&e))
            prop:value=query
        />
        <button type="button" on:click=move |_| search(query.get()) >Search</button>
        <Show when=is_searching fallback=|_| ()>
            <BookSearchResults query on_add_book=move |book| {
                    log!("Copied {:?}", book);
                    on_add_book(book);
                }/>
        </Show>
    }
}

#[component]
fn BookSearchResults<F>(cx: Scope, query: ReadSignal<String>, on_add_book: F) -> impl IntoView
where F: Fn(Book) + 'static + Copy
{
    let search_results = create_resource(cx, query, |query| async move {
        let query = query.clone();
        search_book(&query).await
    });
    view! {cx,
        <Suspense fallback=move || view! {cx, "Loading..."}>
            {move || search_results.read(cx).map(|result| match result {
                Ok(result) => view! {cx,
                    <ul>{result.into_iter().map(|book| view! {cx, <li>
                        {book.title()}
                        <button type="button" on:click=move |_e| {on_add_book(book.clone())}>Add</button>
                        </li>}).collect_view(cx)}</ul>
                }.into_view(cx),
                    Err(err) => view! {cx,
                        <p>"Error" {err.to_string()}</p>
                    }.into_view(cx)
            })}
        </Suspense>
    }
}

#[component]
fn Book(cx: Scope, book: Book) -> impl IntoView {
    view! {cx,
        <div class="book">
            <img class="cover" src={book.cover_src()} alt="Book cover" />
            <div>
                <h3>{book.title()}</h3>
                <p>{book.authors().iter().map(|author| view! {cx, <span>{author}</span>"|"}).collect_view(cx)}</p>
                <p>Description</p>
            </div>
        </div>
    }
}
