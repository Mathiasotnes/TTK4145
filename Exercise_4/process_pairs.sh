#!/bin/bash
script_path="/Users/mathiasotnes/Desktop/TTK4145/Exercises/Exercise_4/process_pairs.sh"
counter_file="/tmp/counter.state" # File to store the counter state

# Set terminated flag
touch "/tmp/terminated.flag"

# For initialization
if [[ "$1" == "--reset" ]]; then
    echo "Resetting script state..."
    rm -f /tmp/terminated.flag /tmp/counter.state # Reset counter file as well
    shift
fi

# Initialize counter
if [ -n "$1" ]; then
    counter=$1
else
    # Check if counter state file exists and read from it
    if [ -f "$counter_file" ]; then
        read counter < "$counter_file"
    else
        counter=0
    fi
fi

# Function to handle termination
cleanup() {
    echo "Terminating... Last count: $counter"
    echo $counter > "$counter_file" # Store the last counter value
    rm -f "/tmp/terminated.flag"
    exit 0
}

# Setup trap for SIGINT
trap cleanup SIGINT

# Check for existence of flag file and wait if it exists
while [ -f "/tmp/terminated.flag" ]; do
    echo "Waiting to overtake..."
    sleep 1
done

# Check if counter state file exists and read from it
if [ -f "$counter_file" ]; then
    read counter < "$counter_file"
else
    counter=0
fi

# Start the backup script in a new terminal
echo "Starting up... Last count: $counter"
osascript <<EOF
tell application "Terminal"
    do script "bash '$script_path' $counter"
end tell
EOF

# Main counting loop
while true; do
    ((counter++))
    echo "Count: $counter"
    sleep 1
done
