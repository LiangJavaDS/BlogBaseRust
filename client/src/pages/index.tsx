import styles from './index.less';
import React from 'react';
import axios from 'axios';
import ArticleInput from "@/components/Article/articleInput";
import { joiningLinAddress } from "@/utils";

export default function IndexPage() {

  const get = () => {
    // 上面的请求也可以这样做
    axios.get(joiningLinAddress("get_all_product"))
      .then(function (response) {
        console.log(response);
      })
      .catch(function (error) {
        console.log(error);
      });
  }

  return (
    <div>
      <ArticleInput />
      <button onClick={get}>点击获取</button>
    </div>
  );
}
