use crate::models::BlogPost;
use dioxus::prelude::*;
use dioxus::logger::tracing::{debug, info, warn, error};

#[server]
pub async fn get_blog_posts() -> Result<Vec<BlogPost>, ServerFnError> {
    let blog_post_1 = BlogPost {
        id : Some(1),
        title: "Cool title 1".to_string(),
        content: "This is content 1".to_string()
    };
    let blog_post_2 = BlogPost {
        id : Some(2),
        title: "Cool title 2".to_string(),
        content: "This is content 2".to_string()
    };

    Ok(vec![blog_post_1,blog_post_2])
}

#[component]
pub fn blog_table_rows(blog_list: Vec<BlogPost>) -> Element {
    if blog_list.is_empty() {
        return rsx! {
            p { "No posts available" }
        }
    }

    rsx! {
        // for post in blog_list.iter() {
        //     tr {
        //         class: "blog_row",
        //         td { "{post.title}" }
        //         //BUG: need to handle if Id is none but rust analyzer gets confused here
        //         // td { "{post.id.}" }
        //         td { "id: totally 1" }
        //     }
        // }
        tr { td {"row 1 col 1"} td { "row 1 col 2" } }
        tr { td {"row 2 col 1"} td { "row 2 col 2" } }
    }
}


#[component]
pub fn BlogTableOfContents() -> Element {
    debug!("Extracting blog table data now");
    let blog_post_list = use_resource(move || async move { get_blog_posts().await });
    let blog_post_list= blog_post_list.read();
    // this is of type
    // GenerationalRef<Ref<'_, Option<Result<Vec<BlogPost>, ServerFnError>>>>
    // but I want it to just be type Result<Vec<BlogPost>, ServerFnError>>
    // or maybe just Vec<BlogPost>
    if blog_post_list.is_some() {
        return rsx! {
            table {
                thead {
                    tr{
                        th { "title" },
                        th { "name" }
                    }
                }
                tbody {
                    tr {
                        class: "post-row",
                        td { "supposed title" },
                        td { "supposed content" }
                    }
                    // blog_table_rows()
                }
            }
        };
    }
    else {
        rsx! {
            p { "no blog posts currently available" }
        }
    }
}
