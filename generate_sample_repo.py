import os
import random
import subprocess
from datetime import datetime, timedelta

def run_git(args, cwd):
    subprocess.check_call(['git'] + args, cwd=cwd)

def main():
    repo_dir = "sample_repo"
    if os.path.exists(repo_dir):
        subprocess.check_call(['rm', '-rf', repo_dir])
    os.makedirs(repo_dir)
    
    run_git(['init'], repo_dir)
    
    users = [
        ("Alice", "alice@example.com"),
        ("Bob", "bob@example.com"),
        ("Charlie", "charlie@example.com"),
        ("Dave", "dave@example.com"),
    ]
    
    start_date = datetime.now() - timedelta(days=30)
    
    # Create 50 random commits
    for i in range(50):
        user_name, user_email = random.choice(users)
        date = start_date + timedelta(days=random.randint(0, 30), hours=random.randint(0, 23))
        date_str = date.strftime("%Y-%m-%dT%H:%M:%S")
        
        # Modify file
        with open(os.path.join(repo_dir, "file.txt"), "a") as f:
            lines = random.randint(1, 20)
            f.write(f"Commit {i} by {user_name}\n" * lines)
            
        run_git(['add', '.'], repo_dir)
        
        env = os.environ.copy()
        env['GIT_AUTHOR_DATE'] = date_str
        env['GIT_COMMITTER_DATE'] = date_str
        env['GIT_AUTHOR_NAME'] = user_name
        env['GIT_AUTHOR_EMAIL'] = user_email
        env['GIT_COMMITTER_NAME'] = user_name
        env['GIT_COMMITTER_EMAIL'] = user_email
        
        subprocess.check_call(['git', 'commit', '-m', f"Commit {i}"], cwd=repo_dir, env=env)
        
    print(f"Generated sample repo at {repo_dir}")

if __name__ == "__main__":
    main()
