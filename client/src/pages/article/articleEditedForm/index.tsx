import React, { FC, useCallback } from "react";
import { Form, Input, Button, message } from 'antd';
import { Article } from "../type";
import { getRequest, postRequest } from "@/utils/index";

type EditedArticleFormProps = {}
/** 文章录入 */
const EditedArticleForm: FC<EditedArticleFormProps> = () => {

    const [form] = Form.useForm();

    const { TextArea } = Input;

    const save = useCallback(async () => {
        const formItemObj = await form.validateFields()
        if (!formItemObj) return
        const { title, content, tag } = formItemObj as Article
        if (!title || !content || !tag) return
        const data = await postRequest("add_blog", { title, tag, content })
        if (!data) return
        message.success("保存成功");
        history.back();
    }, [])

    return <div>
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
            <Form.Item
                label="内容"
                name="content"
                rules={[{ required: true, message: '文章内容不能为空!' }]}
            >
                <TextArea autoSize placeholder="请输入内容" />
            </Form.Item>
        </Form>
        <Button onClick={save}>保存</Button>
    </div>
}
export default EditedArticleForm