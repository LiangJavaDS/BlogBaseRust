import React, { FC, useCallback } from "react";
import { Form, Input, Button } from 'antd';
import axios from 'axios';
import { joiningLinAddress, linAddress } from "@/utils";

type article = {
    title: string,
    content: string
}

type ArticleInputProps = {}
/** 文章录入 */
const ArticleInput: FC<ArticleInputProps> = () => {

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
                    console.log(response);
                })
                .catch(function (error) {
                    console.log(error);
                });
    }, [])

    return <div>
        <Form form={form} name="basic">
            <Form.Item
                label="Title"
                name="title"
                rules={[{ required: true, message: '请输入文章标题!' }]}
            >
                <Input />
            </Form.Item>
            <Form.Item
                label="Content"
                name="content"
                rules={[{ required: true, message: '文章内容不能为空!' }]}
            >
                <TextArea autoSize placeholder="请输入内容" />
            </Form.Item>
        </Form>
        <Button onClick={save}>保存</Button>
    </div>
}
export default ArticleInput