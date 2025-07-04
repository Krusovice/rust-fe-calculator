FROM python:3.12-slim

WORKDIR /app

RUN apt update && apt install -y libfontconfig1

COPY ./api/requirements.txt ./
RUN pip install --no-cache-dir -r requirements.txt

COPY ./api/api_server.py ./
COPY ./target/release/rust-fe-calculator ./

RUN mkdir inputs outputs

EXPOSE 8200

CMD ["uvicorn", "api_server:app", "--host", "0.0.0.0", "--port", "8200"]
