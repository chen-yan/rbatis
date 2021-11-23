#[cfg(test)]
mod test {
    use std::fs::File;
    use std::io::Read;

    use rbatis::executor::RbatisExecutor;
    use rbatis::plugin::page::{Page, PageRequest};
    use rbatis::rbatis::Rbatis;

    use crate::BizActivity;

    #[py_sql("select * from biz_activity where delete_flag = 0")]
    async fn py_ctx_id(rb: &Rbatis) -> Vec<BizActivity> { todo!() }

    #[tokio::test]
    pub async fn test_py_ctx_id() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_ctx_id(&rb).await.unwrap();
        println!("{:?}", a);
    }

    ///select page must have  '?:&PageRequest' arg and return 'Page<?>'
    #[py_sql("select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}")]
    async fn py_select_page(rb: &mut RbatisExecutor<'_,'_>, page_req: &PageRequest, name: &str) -> Page<BizActivity> { todo!() }

    #[tokio::test]
    pub async fn test_py_select_page() {
        fast_log::init_log("requests.log", 1000, log::Level::Info, None, true);
        //use static ref
        let rb = Rbatis::new();
        rb.link("mysql://root:123456@localhost:3306/test")
            .await
            .unwrap();
        let a = py_select_page(&mut rb.as_executor(), &PageRequest::new(1, 10), "test")
            .await
            .unwrap();
        println!("{:?}", a);
    }

    ///Commit the transaction
    #[py_sql("select * from biz_activity where delete_flag = 0
                  if name != '':
                    and name=#{name}")]
    async fn py_sql_tx(rb: &Rbatis, tx_id: &String, name: &str) -> Vec<BizActivity> { todo!() }
}