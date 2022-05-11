import styles from './index.less';
import React from 'react';
import axios from 'axios';

export default function IndexPage() {
  const get = () => {
    // 上面的请求也可以这样做
    axios.get('http://127.0.0.1:8080/')
      .then(function (response) {
        console.log(response);
      })
      .catch(function (error) {
        console.log(error);
      });
  }

  return (
    <div>
      <h1 className={styles.title}>Page index</h1>
      <button onClick={get}>点击获取</button>
    </div>
  );
}
