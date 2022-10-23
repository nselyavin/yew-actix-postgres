import encoding from 'k6/encoding';
import http from 'k6/http';
import { check } from 'k6';

const username = 'test@test.ru';
const password = '1324';
const url = "http://localhost:8080"
const params = {
    headers: {
        'Content-Type': 'application/json',
    },
};

function new_creator(token){
    let payload = JSON.stringify({
        "name": "json baby",
        "license_num": 123
    });

    let res = http.post(
        `${url}/creator/new`,
        payload,
        {
            headers: {
                'Content-Type': 'application/json',
                'Authorization': `Bearer ${token}`
            }
        }
    );

    console.log(res)
    
    check(res, {
        'Creator new: status is 200': (r) => r.status === 200,
    });
    
}

export default function () {
    const credentials = JSON.stringify({
        "email": username,
        "password": password,
    });
    
    let res = http.post(
        `${url}/login`, 
        credentials,
        params
        )
        
    console.log(res);
    check(res, {
        'Auth: status is 200': (r) => r.status === 200,
        'Auth: is authenticated': (r) => r.body.includes("token")
    });
        
    const token = res.json().token;
    console.log(`token: ${token}`);

    new_creator(token);
}