#!/bin/bash

cat ./tamagokill.ascii
cat ./setup.ascii

problem_count=0
incr_pbs () {
  problem_count=$((problem_count+1))
}

cmd_exists () {
  if command -v $1 &> /dev/null
  then
    return 0
  else
    return 1
  fi
}

disp_title() {
  echo ""
  echo "+++"
  echo "#" $1
  echo "+++"
  echo ""
}

step_requirements() {
  
  error_msgs=()
  links_msgs=()

  if cmd_exists "node" && cmd_exists "npm" ;
  then
    node_version=$(node -v)
    npm_version=$(npm -v)
  else
    incr_pbs ;
    node_version=" n/a"
    npm_version=" n/a"
    error_msgs+=("Node.js & npm are mandatory to build & run the Web part." "They are is also needed to run some helpful scripts to manipulate the project as a whole.")
    links_msgs+=("Node.js & npm: https://nodejs.org/en")
  fi

  if cmd_exists "cargo" ;
  then
    cargo_version=$(cargo -V)
  else
    incr_pbs ;
    cargo_version=" n/a"
    error_msgs+=("Cargo is mandatory to build & run the API part. Rust relies on it.")
    links_msgs+=("Cargo: https://doc.rust-lang.org/cargo/getting-started/installation.html")
  fi

  if cmd_exists "docker" ;
  then
    docker_version=$(docker -v)
  else
    incr_pbs ;
    docker_version=" n/a"
    error_msgs+=("Docker is mandatory to run the containerized environment.")
    links_msgs+=("Docker: https://docs.docker.com/")
  fi


  disp_title "Requirements check";

  echo "Node version:    " ${node_version}
  echo "Npm version:     " ${npm_version}
  echo "Cargo version:   " ${cargo_version}
  echo "Docker version:  " ${docker_version}

  echo ""
  if ! [ ${#error_msgs[@]} -eq 0 ];
  then
    echo "--- Problems ---"
    printf '> %s\n' "${error_msgs[@]}"
  else
    echo "You got all the mandatory tools!"
  fi

  if ! [ ${#links_msgs[@]} -eq 0 ];
  then
    echo ""
    echo "--- Downloads ---"
    printf '%s\n' "${links_msgs[@]}"
  fi

}

step_setup_hooks() {
  disp_title "Setup hooks"
  if cp -r ./git-hooks/* ../.git/hooks/ ;
  then
    echo "Hooks installed in the repository."
  else
    incr_pbs ;
    echo "Couldn't install the hooks"
  fi
}

step_requirements ;
step_setup_hooks ;

disp_title "Finalize"
if [ ${problem_count} -eq 0 ]
then
  echo "You are all set!"
else
  echo "You got ${problem_count} errors(s). Review the setup and try again when you solved it!"
fi
