use crate::{storage::*};

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;
use tantivy::ReloadPolicy;
use tempfile::TempDir;

use crate::structs::{ShowsPrintable, ShowPrintable};

pub fn main(
    search_query: &str,
    max_results: &str,
) {
    let max_results = max_results.parse::<usize>().unwrap();
    let search_results = load_show_list();

    let index_path = TempDir::new().unwrap();

    let mut schema_builder = Schema::builder();

    schema_builder.add_text_field("title", TEXT | STORED);
    schema_builder.add_text_field("id", TEXT | STORED);

    let schema = schema_builder.build();

    let index =
        Index::create_in_dir(&index_path, schema.clone())
            .unwrap();

    let mut index_writer =
        index.writer(50_000_000).unwrap();

    let title = schema.get_field("title").unwrap();
    let id = schema.get_field("id").unwrap();

    for show in search_results {
        index_writer.add_document(doc!(
            title => show.name,
        id => show.id as u64));
    }

    index_writer.commit().unwrap();

    let reader = index
        .reader_builder()
        .reload_policy(ReloadPolicy::OnCommit)
        .try_into()
        .unwrap();

    let searcher = reader.searcher();

    let mut query_parser =
        QueryParser::for_index(&index, vec![title]);
    query_parser.set_conjunction_by_default();

    let query =
        query_parser.parse_query(search_query).unwrap();

    let top_docs = searcher
        .search(&query, &TopDocs::with_limit(max_results))
        .unwrap();

    // let search_results = load_show_list()
    //     .into_iter()
    //     .filter(|vec_show| {
    //         vec_show
    //             .name
    //             .to_lowercase()
    //             .contains(&show.to_lowercase())
    //     })
    //     .collect::<Vec<Show>>();

    println!(
        "{}",
        ShowsPrintable {
            shows: top_docs
                .into_iter()
                .map(|(_, doc)| {
                    let doc = searcher.doc(doc).unwrap();
                    ShowPrintable {
                        id: doc
                            .get_first(id)
                            .unwrap()
                            .u64_value()
                            .unwrap()
                            as u32,
                        name: doc
                            .get_first(title)
                            .unwrap()
                            .text()
                            .unwrap()
                            .to_string(),
                    }
                })
                .collect()
        }
    );
}
