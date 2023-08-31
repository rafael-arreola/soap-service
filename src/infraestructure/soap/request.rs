use crate::domain::order::Order;

pub struct SoapRequest {
    pub url: String,
    pub headers: reqwest::header::HeaderMap,
    pub body: Vec<Order>,
}

impl SoapRequest {
    pub fn new(url: String, body: Vec<Order>) -> Self {
        let mut headers = reqwest::header::HeaderMap::new();
        headers.insert("Content-Type", "text/xml; charset=utf-8".parse().unwrap());
        headers.insert("SOAPAction", "KBRELMEDWSaction/AWSPROCESAMEDICOSZAFIRO.Execute".parse().unwrap());
        Self {
            url,
            headers,
            body,
        }
    }

    pub async fn send(&self) -> Result<String, reqwest::Error> {
        let client = reqwest::Client::builder()
            .build()?;

        let request = client.request(reqwest::Method::POST, self.url.as_str())
            .headers(self.headers.clone())
            .body(self.get_body());

        let response = request.send().await?;
        let body = response.text().await?;
        Ok(body)
    }

    pub fn get_body(&self) -> String {
        let mut formatted_orders: Vec<String> = Vec::new();

        for order in &self.body {
            let formatted_order = format!(
                r#"<SDT_PROCESA_MEDICOSZAFIROItem>
                    <MD_CODIGO>{}</MD_CODIGO>
                    <CMARCA>{}</CMARCA>
                    <FECHAINI>{}</FECHAINI>
                    <FECHAFIN>{}</FECHAFIN>
                </SDT_PROCESA_MEDICOSZAFIROItem>"#,
                order.codigo, order.marca, order.fecha_ini, order.fecha_fin
            );
            formatted_orders.push(formatted_order);
        }

        let request = format!(
            r#"<?xml version="1.0" encoding="utf-8"?>
            <soap:Envelope xmlns:soap="http://schemas.xmlsoap.org/soap/envelope/">
              <soap:Body>
                  <wsprocesamedicoszafiro.Execute xmlns="KBRELMEDWS">
                        <Sdt_procesa_medicoszafiro>
                        {}
                        </Sdt_procesa_medicoszafiro>
                    </wsprocesamedicoszafiro.Execute>
                </soap:Body>
            </soap:Envelope>"#
            , formatted_orders.join("")
        );
        return request;
    }
}