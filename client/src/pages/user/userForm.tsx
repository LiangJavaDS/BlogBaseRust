import React, { FC, useCallback } from "react";
import { Form, Input, Button, message } from 'antd';
import { getRequest, postRequest } from "@/utils/index";

type UserFormProps = {}
/** 用户录入 */
const UserForm: FC<UserFormProps> = () => {

    const [form] = Form.useForm();

    const save = useCallback(async () => {
        const formItemObj = await form.validateFields()
        if (!formItemObj) return
        const {
            username,
            password,
            checkedPassword,
            email
        } = formItemObj as addUser
        if (!username || !password || !checkedPassword || !email) return
        // 两次密码是否相同
        if (checkedPassword !== password) return
        const data = await postRequest("add_user", { username, password, email })
        if (!data) return
        message.success("保存成功");
        // history.back();
    }, [])

    return <div>
        <Form form={form} name="basic">
            <Form.Item
                label="用户名"
                name="username"
                rules={[{ required: true, message: '请输入用户名!' }]}
            >
                <Input />
            </Form.Item>
            <Form.Item
                label="密码"
                name="password"
                rules={[{ required: true, message: '请输入密码!' }]}
            >
                <Input />
            </Form.Item>
            <Form.Item
                label="确认密码"
                name="checkedPassword"
                rules={[{ required: true, message: '请再次输入密码!' }]}
            >
                <Input />
            </Form.Item>
            <Form.Item
                label="邮箱"
                name="email"
                rules={[{ required: true, message: '请输入邮箱!' }]}
            >
                <Input placeholder="请输入内容" />
            </Form.Item>
        </Form>
        <Button onClick={save}>保存</Button>
    </div>
}
export default UserForm