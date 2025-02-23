function sleep(seconds)
    local start_time = os.clock()

    while os.clock() - start_time < seconds do end
end

print("hows it goin")
sleep(0.5)
print("welcome to the test program")