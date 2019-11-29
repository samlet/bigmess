#[macro_use]
extern crate tantivy;
extern crate sagas_greeter;

use tantivy::collector::TopDocs;
use tantivy::query::QueryParser;
use tantivy::schema::*;
use tantivy::Index;

use sagas_greeter::jieba_tokenizer::*;

fn main() -> tantivy::Result<()>{
    cn()
}

fn cn() -> tantivy::Result<()>{
    // Start building a new schema.
    let mut schema_builder = Schema::builder();

    // Create a new field `body` using TinySegmenter as the tokenizer.
    let text_field_indexing = TextFieldIndexing::default()
        .set_tokenizer("jiebaseg")
        .set_index_option(IndexRecordOption::WithFreqsAndPositions);
    let text_options = TextOptions::default()
        .set_indexing_options(text_field_indexing)
        .set_stored();
    let body = schema_builder.add_text_field("body", text_options);

    let schema = schema_builder.build();

    // Create a new index from the schema.
    let index = Index::create_in_ram(schema.clone());

    // Register TinySegmenterTokenizer as "tinyseg".
    index.tokenizers().register("jiebaseg", JiebaTokenizer {});

    let mut index_writer = index.writer(50_000_000)?;

    index_writer.add_document(doc!(
        body => "塑料钢笔",
    ));
    index_writer.add_document(doc!(
        body => r#"塑料铅笔"#,
    ));
    index_writer.add_document(doc!(
        body => r#"北京大学"#,
    ));
    index_writer.commit()?;

    let reader = index.reader()?;
    let searcher = reader.searcher();

    let query_parser = QueryParser::for_index(&index, vec![body]);

    // Search for "塑料", which is contained in the 2nd and 3rd document.
    let query = query_parser.parse_query("塑料")?;

    // Sort results by relavance and print them.
    let top_docs = searcher.search(&query, &TopDocs::with_limit(10))?;
    for (_, doc_address) in top_docs {
        let retrieved_doc = searcher.doc(doc_address)?;
        println!("{}", schema.to_json(&retrieved_doc));
    }

    Ok(())
}
