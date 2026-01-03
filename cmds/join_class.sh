TOKEN=$1
shift
IDS=$(printf "[]=%s " "$@")
http :3000/students_classes 'Authorization: Bearer '$TOKEN $IDS
