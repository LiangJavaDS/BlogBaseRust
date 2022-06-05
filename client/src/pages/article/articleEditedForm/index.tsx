import Editor from "react-markdown-editor-lite";
import ReactMarkdown from "react-markdown";
import "react-markdown-editor-lite/lib/index.css";
import React, { FC, useCallback } from "react";
import { Form, Input, Button, message } from 'antd';
import { Article } from "../type";
import { postRequest } from "@/utils/index";
import styles from './index.less';

/** 文章录入-MarkDown */
const EditedArticle = () => {
    const [form] = Form.useForm();
    const mdEditor = React.useRef(null);
    const [value, setValue] = React.useState("");

    const save = useCallback(async () => {
        const formItemObj = await form.validateFields()
        if (!formItemObj) return
        const { title, tag } = formItemObj as Article
        if (!title || !value || !tag) return
        const data = await postRequest("add_blog", { title, tag, content: value })
        if (!data) return
        message.success("保存成功");
        history.back();
    }, [value])

    const handleEditorChange = useCallback(({ html, text }: { html: string, text: string }) => {
        // 过滤
        // const newValue = text.replace(/\d/g, "");
        // console.log(newValue);
        setValue(text);
    }, []);

    return (
        <div>
            <Form form={form} name="basic">
                <Form.Item
                    label="标题"
                    name="title"
                    rules={[{ required: true, message: '请输入文章标题!' }]}
                >
                    <Input />
                </Form.Item>
                <Form.Item
                    label="标签"
                    name="tag"
                    rules={[{ required: true, message: '请输入文章标签!' }]}
                >
                    <Input />
                </Form.Item>
                <Editor
                    className={styles.editor}
                    ref={mdEditor}
                    value={value}
                    onChange={handleEditorChange}
                    renderHTML={text => {
                        return <ReactMarkdown children={text} />
                    }}
                />
            </Form>
            <button onClick={save}>保存</button>
        </div>
    );
}
export default EditedArticle