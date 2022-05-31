import React, { FC, useCallback, useEffect } from "react";
import { Form, Input, Button } from 'antd';
import axios from 'axios';
import { joiningLinAddress, linAddress } from "@/utils";

type article = {
    title: string,
    content: string
}

type ReadOnlyArticleProps = {}
/** 文章录入 */
const ReadOnlyArticle: FC<ReadOnlyArticleProps> = () => {

    const [form] = Form.useForm();

    const { TextArea } = Input;

    const save = useCallback(async () => {
        // toDo post请求
        const formItemObj = await form.validateFields()
        console.log(formItemObj);
        if (!formItemObj) return
        const { title, content } = formItemObj as article
        if (title && content)
            axios.post(joiningLinAddress("add_product"), {
                title,
                name: content
            })
                .then(function (response) {
                    console.log('7878', response);
                })
                .catch(function (error) {
                    console.log(error);
                });
    }, [])


    useEffect(() => {
        console.log("我加载了")
    }, [])

    return <div>
        <h1>我是详情1</h1>
    </div>
}
export default ReadOnlyArticle