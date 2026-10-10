# 概要
指定した単語から辞書形式のマークダウンファイルを作ります。  
# 目的  
英単語覚えるとき英英辞書のほうがわかりやすいと感じたので覚えたい単語を印刷できるデータにしたかった。
# 仕様
- データソース:Merriam Webster dictionaryのAPI  
- 入力:input.jsonでAPIキーと単語を指定
- 出力:辞書形式に整形されたproduct.md
# 使用方法  
1. プロジェクト直下にinput.jsonという名前のjsonを置いAPIキーと調べたい単語の配列を書く。
2. cargo runで実行する。  
3. 必要に応じてpdfファイルなどに変換する。

input.jsonの例
```json
{
    "api_key":"YOUR_API_KEY",  
    "words":[          
        "words",  
        "you",  
        "want",  
        "to",  
        "memorize"  
    ]
}

```
YOUR_API_KEYにはhttps://dictionaryapi.com/products/api-collegiate-dictionary で取得したAPIキーを指定してください。  
pdfなどへの変換はVSCodeの拡張機能などが使用できます。

