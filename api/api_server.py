from fastapi import FastAPI, UploadFile, File
from fastapi.responses import FileResponse
import subprocess
import os
import shutil

app = FastAPI()

@app.post("/run/")
def run_fe_model(
    keypoints: UploadFile = File(...),
    materials: UploadFile = File(...),
    connections: UploadFile = File(...),
    bcs: UploadFile = File(...),
    pointloads: UploadFile = File(...),
):
    # Ensure input/output folders exist
    os.makedirs("inputs", exist_ok=True)
    os.makedirs("outputs", exist_ok=True)

    # Save all files with correct filenames
    input_files = {
        "keypoints.txt": keypoints,
        "materials.txt": materials,
        "connections.txt": connections,
        "bcs.txt": bcs,
        "pointloads.txt": pointloads,
    }

    for name, file in input_files.items():
        with open(f"inputs/{name}", "wb") as f:
            shutil.copyfileobj(file.file, f)

    # Run Rust binary
    subprocess.run(["./rust-fe-calculator"])

    output_result = './outputs/keypoint_result_data.json'
    return FileResponse(output_result, media_type="application/json")
