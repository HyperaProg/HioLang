/*
 * HioClib Library Example - Networking in Go
 * HTTP and network utilities for Hiolang
 */

package main

import "C"
import (
    "bytes"
    "io/ioutil"
    "net/http"
    "time"
)

//-------------------------
//-------------------------

//export HioHttpGet_c
func HioHttpGet_c(url *C.char) *C.char {
    goUrl := C.GoString(url)

    client := &http.Client{
        Timeout: time.Second * 10,
    }

    resp, err := client.Get(goUrl)
    if err != nil {
        return C.CString(err.Error())
    }
    defer resp.Body.Close()

    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        return C.CString(err.Error())
    }

    return C.CString(string(body))
}

//export HioHttpPost_c
func HioHttpPost_c(url *C.char, data *C.char) *C.char {
    goUrl := C.GoString(url)
    goData := C.GoString(data)

    client := &http.Client{
        Timeout: time.Second * 10,
    }

    resp, err := client.Post(
        goUrl,
        "application/json",
        bytes.NewBufferString(goData),
    )
    if err != nil {
        return C.CString(err.Error())
    }
    defer resp.Body.Close()

    body, err := ioutil.ReadAll(resp.Body)
    if err != nil {
        return C.CString(err.Error())
    }

    return C.CString(string(body))
}

//export HioGetTimestamp_c
func HioGetTimestamp_c() C.longlong {
    return C.longlong(time.Now().Unix())
}

//export HioSleep_c
func HioSleep_c(ms C.longlong) {
    time.Sleep(time.Duration(ms) * time.Millisecond)
}

//-------------------------
//-------------------------
func main() {}