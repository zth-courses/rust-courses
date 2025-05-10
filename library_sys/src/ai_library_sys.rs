use anyhow::{Ok, Result};
use clap::{Parser, Subcommand};
use dialoguer::{Input};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::fs;
use std::path::Path;

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[command(subcommand)]
    command: Commands,
}

#[derive(Subcommand)]
enum Commands {
    Add,
    Remove,
    List,
    Borrow,
    Return,
}

#[derive(Serialize, Deserialize, Clone)]
struct Book {
    title: String,
    author: String,
    isbn: String,
    available: bool,
}

struct Library {
    books: HashMap<String, Book>,
}

impl Library {
    fn new() -> Self {
        Self {
            books: HashMap::new(),
        }
    }
    // 从文件加载数据
    fn load_from_file() -> Result<Self> {
      let path = "library_data.json";
      if Path::new(path).exists() {
          let content = fs::read_to_string(path)?;
          let books: HashMap<String, Book> = serde_json::from_str(&content)?;
          Ok(Self { books })
      } else {
          Ok(Self::new())
      }
  }

  // 保存数据到文件
  fn save_to_file(&self) -> Result<()> {
      let content = serde_json::to_string_pretty(&self.books)?;
      fs::write("library_data.json", content)?;
      Ok(())
  }
    fn add_book(&mut self, book: Book) -> Result<()> {
        self.books.insert(book.isbn.clone(), book);
        self.save_to_file()?;
        Ok(())
    }

    fn remove_book(&mut self, isbn: &str) -> Result<Option<Book>> {
      let book = self.books.remove(isbn);
      if book.is_some() {
          self.save_to_file()?;
      }
      Ok(book)
  }

    fn list_books(&self) {
        if self.books.is_empty() {
            println!("图书馆目前没有藏书");
            return;
        }
        for book in self.books.values() {
            println!(
                "书名: {}, 作者: {}, ISBN: {}, 状态: {}",
                book.title,
                book.author,
                book.isbn,
                if book.available { "可借" } else { "已借出" }
            );
        }
    }
    fn borrow_book(&mut self, isbn: &str) -> Result<()> {
      if let Some(book) = self.books.get_mut(isbn) {
          if book.available {
              book.available = false;
              self.save_to_file()?;
              println!("借书成功！");
              Ok(())
          } else {
              anyhow::bail!("该书已被借出")
          }
      } else {
          anyhow::bail!("找不到该书籍")
      }
  }


  fn return_book(&mut self, isbn: &str) -> Result<()> {
    if let Some(book) = self.books.get_mut(isbn) {
        if !book.available {
            book.available = true;
            self.save_to_file()?;
            println!("还书成功！");
            Ok(())
        } else {
            anyhow::bail!("该书已在馆内")
        }
    } else {
        anyhow::bail!("找不到该书籍")
    }
}
}

fn main() -> Result<()> {
  let cli = Cli::parse();
  let mut library = Library::load_from_file()?;

  match cli.command {
      Commands::Add => {
          let title: String = Input::new().with_prompt("请输入书名").interact_text()?;
          let author: String = Input::new().with_prompt("请输入作者").interact_text()?;
          let isbn: String = Input::new().with_prompt("请输入ISBN").interact_text()?;
          
          let book = Book {
              title,
              author,
              isbn: isbn.clone(),
              available: true,
          };
          library.add_book(book)?;
          println!("添加成功！");
      }
      Commands::Remove => {
          let isbn: String = Input::new().with_prompt("请输入要删除的书籍ISBN").interact_text()?;
          if library.remove_book(&isbn)?.is_some() {
              println!("删除成功！");
          } else {
              println!("找不到该书籍");
          }
      }
      Commands::List => {
          library.list_books();
      }
      Commands::Borrow => {
          let isbn: String = Input::new().with_prompt("请输入要借阅的书籍ISBN").interact_text()?;
          if let Err(e) = library.borrow_book(&isbn) {
              println!("借书失败：{}", e);
          }
      }
      Commands::Return => {
          let isbn: String = Input::new().with_prompt("请输入要归还的书籍ISBN").interact_text()?;
          if let Err(e) = library.return_book(&isbn) {
              println!("还书失败：{}", e);
          }
      }
  }

  Ok(())
}

// 要运行这个文件，可以使用命令：
// cargo run --bin ai_library_sys