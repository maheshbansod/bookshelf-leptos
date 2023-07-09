use gloo_storage::Storage;
use leptos::*;

mod bookshelf;
use bookshelf::*;
fn main() {
    mount_to_body(move |cx| view! {cx, <Bookshelf /> })
}

const BOOKSHELF_STORAGE_KEY: &str = "BOOKSHELF_STORAGE_KEY";

#[component]
fn Bookshelf(cx: Scope) -> impl IntoView {
    let (bookshelf, set_bookshelf) = create_signal(
        cx,
        gloo_storage::LocalStorage::get(BOOKSHELF_STORAGE_KEY).unwrap_or(Bookshelf::new()),
    );
    let on_add_book = move |book: Book| {
        set_bookshelf.update(|bookshelf| bookshelf.add_book(book));
    };
    create_effect(cx, move |_| {
        if let Err(err) = gloo_storage::LocalStorage::set(BOOKSHELF_STORAGE_KEY, bookshelf.get()) {
            error!("Couldn't write to LocalStorage.\n{err:?}");
        }
    });
    view! {cx,
        <BookSearch on_add_book=on_add_book/>
        <h1>"Bookshelf"</h1>
        <hr/>
        <BookList books=Signal::derive(cx, move || bookshelf.get().books().to_vec()) />
    }
}

#[component]
fn BookList(cx: Scope, #[prop(into)] books: Signal<Vec<Book>>) -> impl IntoView {
    let (query, set_query) = create_signal(cx, String::new());
    let books = move || {
        books
            .get()
            .clone()
            .into_iter()
            .filter(|book| {
                book.title()
                    .clone()
                    .to_lowercase()
                    .contains(&query.get().to_lowercase())
            })
            .collect::<Vec<_>>()
    };
    view! {cx,
        <input 
            type="text" 
            placeholder="Search books in the bookshelf"
            on:input=move |e| set_query(event_target_value(&e))
            prop:value=query
        />
        <For
            each=move || books()
            key=|book| book.id().clone()
            view=move |cx, book| view! {cx, <Book book />}
        />
    }
}

#[component]
fn BookSearch<F>(cx: Scope, on_add_book: F) -> impl IntoView
where
    F: Fn(Book) + 'static + Copy,
{
    let (query, set_query) = create_signal(cx, String::new());
    let (is_searching, set_is_searching) = create_signal(cx, false);
    let search = move |_query: String| {
        set_is_searching.set(true);
    };
    view! {cx,
        <input
            type="search"
            placeholder="Search books"
            on:input=move |e| {
                set_is_searching.set(false);
                set_query(event_target_value(&e));
            }
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
where
    F: Fn(Book) + 'static + Copy,
{
    let search_results = create_resource(
        cx,
        || (),
        move |_| async move {
            let query = query.get_untracked();
            search_book(&query).await
        },
    );
    view! {cx,
        <Suspense fallback=move || view! {cx, "Loading..."}>
            {move || search_results.read(cx).map(|result| match result {
                Ok(result) => view! {cx,
                    <p>"Search results for '" {query} "'"</p>
                    <ul>{result.into_iter().map(|book| view! {cx, <li>
                        {book.title()}
                        <button type="button" on:click=move |_e| {on_add_book(book.clone())}>Add</button>
                        </li>}).collect_view(cx)}</ul>
                }.into_view(cx),
                    Err(err) => view! {cx,
                        <p>"Error " {err.to_string()}</p>
                    }.into_view(cx)
            })}
        </Suspense>
    }
}

#[component]
fn Book(cx: Scope, book: Book) -> impl IntoView {
    view! {cx,
        <div class="book">
            <img class="cover" src={book.cover_src().clone().map(|c| c.get().clone()).unwrap_or("x".to_string())} alt="Book cover" />
            <div class="book_info">
                <span class="book_title">{book.title()}</span>
                <span class="first_publish_year">
                    {book
                        .first_publish_year()
                        .map(|y| y.to_string()).unwrap_or("".to_string())
                    }
                </span>
                <p class="authors">{book.authors().iter().enumerate().map(|(i,author)| view! {cx,
                    <span class="author">{author}</span>
                    {(i > 1 && i != book.authors().len()).then_some(" | ")}
                }).collect_view(cx)}</p>
            </div>
        </div>
    }
}
