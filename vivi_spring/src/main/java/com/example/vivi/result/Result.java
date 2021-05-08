package com.example.vivi.result;

import java.util.HashMap;
import java.util.Map;

public class Result{
    //响应码
    private int code;
    //数据
    private Map<String,Integer> intData = new HashMap<>();

    public Result(int code){
        this.code=code;
    }

    public Result(String key, Integer data){ this.intData.put(key,data); }

    public int getCode() {
        return code;
    }

    public Map<String, Integer> getIntData() {
        return intData;
    }

    public void setIntData(Map<String, Integer> intData) {
        this.intData = intData;
    }

    public void setCode(int code) {
        this.code = code;
    }
}