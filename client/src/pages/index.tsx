import styles from './index.less';
import React from 'react';
import axios from 'axios';
import ArticleInput from "@/components/Article/articleInput";
import { linAddress } from "@/utils.ts";

export default function IndexPage() {

  const get = () => {
    // 上面的请求也可以这样做
    axios.get(linAddress)
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
      <h1 className={styles.title}>Page index</h1>
      <button onClick={get}>点击获取</button>
    </div>
  );
}
