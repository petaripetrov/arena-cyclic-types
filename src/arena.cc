#include "thesis/include/arena.h"
#include <iostream>

template <typename T>
class Node
{
public:
    T data;
    Node *parent;
    Node *right;
    Node *left;

    Node(T init)
    {
        this->data = init;
        this->parent = NULL;
        this->right = NULL;
        this->left = NULL;
    }

    ~Node()
    {
        if (this->right != NULL)
        {
            free(this->right);
            this->right = NULL;
        }

        if (this->left != NULL)
        {
            free(this->left);
            this->left = NULL;
        }

        if (this->parent != NULL)
        {
            free(this->parent);
            this->parent = NULL;
        }
    }
};

template <typename T>
class List
{
private:
    Node<T> *head;
    Node<T> *tail;

public:
    List()
    {
        this->head = NULL;
        this->tail = NULL;
    }

    ~List()
    {
        Node<T> *temp = this->head;
        Node<T> *end = NULL;

        while (temp != NULL)
        {
            if (temp->right == NULL)
            {

                end = temp;
                break;
            }

            temp = temp->right;
        }

        temp = end;

        while (temp != NULL)
        {
            Node<T> *inner = temp;
            temp = temp->left;
            free(inner);
        }
    }

    void push(Node<T> *data);
    void push_constant(Node<T> *data);
    void print();

    void null_top()
    {
        this->head = NULL;
        this->tail = NULL;
    }
};

template <typename T>
void List<T>::push(Node<T> *data)
{

    if (this->head == NULL)
    {
        this->head = data;
        return;
    }

    Node<T> *temp = this->head;

    while (temp != NULL)
    {
        if (temp->right == NULL)
        {
            data->left = temp;
            temp->right = data;
            break;
        }

        temp = temp->right;
    }
}

template <typename T>
void List<T>::push_constant(Node<T> *data)
{
    if (this->head == NULL)
    {
        this->head = data;
        return;
    }

    if (this->tail == NULL)
    {
        this->tail = data;
        this->head->right = data;
    }
    else
    {
        this->tail->right = data;
        this->tail = data;
    }
}

template <typename T>
void List<T>::print()
{
    Node<T> *temp = this->head;

    while (temp != NULL)
    {
        std::cout << "Node with value :: " << temp->data << "\n";
        temp = temp->right;
    }
}

template <typename T>
class Tree
{
private:
    Node<T> *root;

    void print_in_order(Node<T> *node);

public:
    Tree()
    {
        this->root = NULL;
    }

    ~Tree()
    {
        if (this->root != NULL)
        {
            delete (this->root);
        }
    }

    void push(Node<T> *data);
    void print();

    void null_root()
    {
        this->root = NULL;
    }
};

template <typename T>
void Tree<T>::push(Node<T> *data)
{
    if (this->root == NULL)
    {
        this->root = data;

        return;
    }

    Node<T> *temp = this->root;

    while (temp != NULL)
    {
        if (data->data < temp->data || data->data == temp->data)
        {
            if (temp->left == NULL)
            {
                data->parent = temp;
                temp->left = data;
                break;
            }

            temp = temp->left;
        }

        if (data->data > temp->data)
        {
            if (temp->right == NULL)
            {
                data->parent = temp;
                temp->right = data;
                break;
            }

            temp = temp->right;
        }
    }
}

template <typename T>
void Tree<T>::print()
{
    Node<T> *base = this->root;

    std::cout << "Tree:\n";
    this->print_in_order(base);
    std::cout << "\n";
}

template <typename T>
void Tree<T>::print_in_order(Node<T> *node)
{
    if (node != NULL)
    {
        this->print_in_order(node->left);
        std::cout << node->data << " ";
        this->print_in_order(node->right);
    }
}

void benchmark_linked_list_arena(size_t n)
{
    Arena<Node<size_t>> *arena = new Arena<Node<size_t>>(n);
    List<size_t> *list = new List<size_t>;

    for (size_t i = 0; i < n; i++)
    {
        Node<size_t> *val = new (arena->allocate()) Node<size_t>(i);
        list->push(val);
    }

    delete (arena);
    list->null_top();
    free(list);
}

void benchmark_tree_arena(size_t n, rust::Vec<uint32_t> arr)
{
    Arena<Node<size_t>> *arena = new Arena<Node<size_t>>(n);
    Tree<size_t> *tree = new Tree<size_t>;

    for (auto num : arr)
    {
        Node<size_t> *val = new (arena->allocate()) Node<size_t>(num);
        tree->push(val);
    }

    delete (arena);
    tree->null_root();
}

void benchmark_linked_list_manual(size_t n)
{
    List<size_t> *list = new List<size_t>;

    for (size_t i = 0; i < n; i++)
    {
        Node<size_t> *val = new Node<size_t>(i);
        list->push(val);
    }

    delete (list);
}

void benchmark_tree_manual(rust::Vec<uint32_t> arr)
{
    Tree<size_t> *tree = new Tree<size_t>;

    for (auto num : arr)
    {
        Node<size_t> *val = new Node<size_t>(num);
        tree->push(val);
    }

    delete (tree);
}
